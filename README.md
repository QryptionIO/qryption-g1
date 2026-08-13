# Qryption G1 — Cryptographic Proof of Concept

PoC que demuestra una integración funcional de **ML-DSA** dentro de una
primitiva mínima de autorización de operaciones, con controles básicos
contra modificación (tampering), reutilización (replay) y uso fuera de
contexto.

Este PoC **no** es Qryption, no es la Qryption Key, no incluye hardware,
blockchain, ni ningún componente de producto. Ver la sección "Qué NO
demuestra este PoC" más abajo.

## Requisitos

- Rust 1.97.1 o superior (edición 2021)
- Sin dependencias externas de C ni toolchain adicional

## Dependencia criptográfica

- Crate: [`ml-dsa`](https://crates.io/crates/ml-dsa) v0.1.1 (RustCrypto,
  implementación pura en Rust de FIPS 204, sin auditar)
- Parámetro utilizado: **ML-DSA-65** (nivel de seguridad 3, 192 bits)
- Tamaños confirmados por test (`prints_key_and_signature_sizes`):
  - Clave pública (verifying key): 1952 bytes
  - Firma: 3309 bytes
  - Clave privada (signing key): 4032 bytes según especificación oficial
    FIPS 204 (no exportada directamente por la API de esta versión del
    crate por razones de manejo seguro de material sensible)

## Cómo compilar
```
   cargo build
```
## Cómo ejecutar los tests
```
   cargo test
```
## Qué demuestra cada test

| Test | Qué demuestra |
|---|---|
| `key_generation.rs::generates_nonempty_keys` | Generación correcta de un par de claves ML-DSA-65 |
| `key_generation.rs::prints_key_and_signature_sizes` | Tamaños reales de clave pública y firma, verificados contra FIPS 204 |
| `signing.rs::signs_authorization_payload` | Firma de un payload de autorización estructurado (no un mensaje arbitrario) |
| `verification.rs::happy_path_generate_sign_verify` | Camino feliz completo: generar → firmar → verificar → VALID |
| `tamper.rs::tampered_payload_is_rejected` | Modificar un campo del payload tras firmarlo invalida la firma |
| `replay.rs::second_use_of_same_nonce_is_rejected` | Una autorización con firma válida no puede reutilizarse dos veces (protección a nivel de protocolo, no de criptografía) |
| `context.rs::signature_bound_to_original_context_only` | Una firma válida en un contexto no es válida en otro distinto |

## Arquitectura
src/
├── lib.rs — declaración de módulos
├── keys.rs — generación de identidad (par de claves ML-DSA-65)
├── authorization.rs — payload estructurado + serialización determinista
├── signing.rs — firma del payload serializado
├── verification.rs — verificación criptográfica (VALID/INVALID)
└── protocol.rs — capa de protocolo: registro de nonces usados
para prevenir replay (fuera de la criptografía)

## Decisiones tomadas

- Se eligió `ml-dsa` (RustCrypto, Rust puro) en vez de bindings sobre
  `liboqs`/PQClean, para minimizar dependencias de toolchain C y facilitar
  la reproducibilidad en cualquier máquina con Rust instalado.
- Se usó `serde_json` para la serialización canónica del payload de
  autorización antes de firmar, priorizando claridad sobre optimización de
  tamaño en esta fase.
- La protección contra replay se implementa explícitamente como lógica de
  protocolo (`AuthorizationLedger`), no como parte de la firma, siguiendo
  la distinción que marca el brief: la criptografía por sí sola no impide
  el replay.

## Limitaciones conocidas

- El crate `ml-dsa` usado no ha sido auditado de forma independiente
  (advertencia explícita de sus propios autores).
- El registro de nonces (`AuthorizationLedger`) es en memoria y no
  persiste entre ejecuciones — suficiente para demostrar la propiedad,
  pero no apto para producción.
- No se ha probado el manejo de expiración (`expires_at`) con lógica de
  tiempo real; el campo existe en el payload pero no se valida todavía.

## Qué NO demuestra este PoC

Si todos los tests pasan, **no** podemos concluir que:

- Qryption es seguro.
- Qryption es resistente a todos los ataques cuánticos.
- Tenemos una solución empresarial lista.
- Nuestro hardware es seguro.
- Hemos demostrado el protocolo definitivo.

Lo único que este PoC demuestra es: **una integración funcional de
ML-DSA dentro de una primitiva mínima de autorización, incluyendo
controles básicos contra modificación, replay y uso fuera de contexto.**

## Pendiente / siguiente gate

- Validación de expiración (`expires_at`) contra tiempo real.
- Persistencia del registro de nonces.
- Decisión sobre el siguiente gate del proyecto tras revisión de este PoC.
