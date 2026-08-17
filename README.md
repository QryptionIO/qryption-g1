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

```text
cargo build
```

## Cómo ejecutar los tests

```text
cargo test --locked
```

## Qué demuestra cada test

| Test | Qué demuestra |
|---|---|
| `key_generation.rs::generates_nonempty_keys` | Generación correcta de un par de claves ML-DSA-65 |
| `key_generation.rs::prints_key_and_signature_sizes` | Tamaños reales de clave pública y firma, verificados contra FIPS 204 |
| `signing.rs::signs_authorization_payload` | Firma de una estructura `Authorization` de siete campos |
| `verification.rs::happy_path_generate_sign_verify` | Camino feliz completo: generar → firmar → verificar → VALID |
| `tamper.rs::tampered_payload_is_rejected` | Modificar un campo de `Authorization` tras firmarlo invalida la firma |
| `replay.rs::second_use_of_same_nonce_is_rejected` | Una autorización con firma válida no puede reutilizarse dos veces (protección a nivel de protocolo, no de criptografía) |
| `context.rs::signature_bound_to_original_context_only` | Una firma válida en un contexto no es válida en otro distinto |
| `canonical_vectors.rs::canonical_vectors_match_frozen_g1_g3_format` | Los siete vectores congelados producen exactamente los bytes esperados para compatibilidad G1-G3 |

## Arquitectura

```text
src/
├── lib.rs           — declaración de módulos
├── keys.rs          — generación de identidad (par de claves ML-DSA-65)
├── authorization.rs — esquema Authorization de siete campos y serialización determinista
├── signing.rs       — firma de Authorization serializada
├── verification.rs  — verificación criptográfica (VALID/INVALID)
└── protocol.rs      — registro de nonces usados para prevenir replay
                       (fuera de la criptografía)

tests/
├── canonical_vectors.rs — verificación byte a byte del formato congelado
└── fixtures/
    └── canonical_vectors.json — vectores de compatibilidad G1-G3

docs/
└── adr/
    └── 0001-formato-canonico-autorizacion.md — decisión técnica del formato
```

## Formato canónico de autorización G1-G3

El mensaje firmado no sigue una abstracción genérica
`payload + context + nonce`. Se serializa la estructura `Authorization`
completa mediante `serde_json::to_vec(self)`.

El formato queda congelado con estos campos, en este orden:

1. `operation: String`
2. `amount: String`
3. `currency: String`
4. `destination: String`
5. `nonce: u64`
6. `expires_at: u64`
7. `context: String`

Representación literal, sin espacios añadidos:

```text
{"operation":<JSON_STRING>,"amount":<JSON_STRING>,"currency":<JSON_STRING>,"destination":<JSON_STRING>,"nonce":<U64_DECIMAL>,"expires_at":<U64_DECIMAL>,"context":<JSON_STRING>}
```

Las versiones de referencia están fijadas en:

- `serde 1.0.229`
- `serde_json 1.0.151`

La definición, el alcance y los riesgos de compatibilidad están documentados
en [ADR 0001](docs/adr/0001-formato-canonico-autorizacion.md).

Los [vectores canónicos](tests/fixtures/canonical_vectors.json) se verifican
byte a byte mediante
[canonical_vectors.rs](tests/canonical_vectors.rs).

Para G3, reproducir este comportamiento en Rust `no_std` requiere comprobar
`serde_json` con `alloc`, configurar un allocator y medir el consumo conjunto
de flash, heap, RAM y stack con ML-DSA-65 en el target ARM.

## Decisiones tomadas

- Se eligió `ml-dsa` (RustCrypto, Rust puro) en vez de bindings sobre
  `liboqs`/PQClean, para minimizar dependencias de toolchain C y facilitar
  la reproducibilidad en cualquier máquina con Rust instalado.
- Se usó `serde_json` para la serialización canónica de `Authorization`
  antes de firmar, priorizando claridad sobre optimización de tamaño en
  esta fase.
- Se fijaron `serde 1.0.229` y `serde_json 1.0.151` para mantener estable
  la representación utilizada por G1 y reproducirla en G3.
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
  tiempo real; el campo existe en `Authorization`, pero no se valida
  todavía.
- El formato no contiene un campo explícito de dominio o versión. El
  hallazgo queda registrado como pendiente no bloqueante.
- La compatibilidad de `serde_json` con `no_std + alloc` y su consumo
  conjunto de memoria con ML-DSA-65 deben validarse en el target ARM de G3.

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
- Spike de G3 con ML-DSA-65 y seed fijo sobre el target ARM.
- Comprobación de `serde_json` con `no_std + alloc`.
- Reproducción byte a byte de los siete vectores en G3.
- Medición combinada de flash, heap, RAM y stack.
- Revisión futura del campo explícito de dominio o versión.