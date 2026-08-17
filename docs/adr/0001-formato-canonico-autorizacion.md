# ADR 0001: Formato canónico de autorización G1-G3

- Estado: Aceptada
- Fecha: 2026-08-18
- Alcance: Qryption G1 y compatibilidad con el firmware G3

## Contexto

La documentación inicial describía el mensaje firmado mediante la abstracción
genérica `payload + context + nonce`. La implementación real de G1 no contiene
un campo `payload` binario o genérico. Firma una estructura `Authorization`
compuesta por siete campos.

Los bytes entregados a ML-DSA-65 se obtienen exclusivamente mediante
`Authorization::canonical_bytes()`, que ejecuta
`serde_json::to_vec(self)`.

## Decisión

El formato canónico de G1 y G3 queda congelado con estos campos, en este orden y
con estos tipos:

1. `operation: String`
2. `amount: String`
3. `currency: String`
4. `destination: String`
5. `nonce: u64`
6. `expires_at: u64`
7. `context: String`

La representación literal es:

```text
{"operation":<JSON_STRING>,"amount":<JSON_STRING>,"currency":<JSON_STRING>,"destination":<JSON_STRING>,"nonce":<U64_DECIMAL>,"expires_at":<U64_DECIMAL>,"context":<JSON_STRING>}
```

No se añaden espacios. Las cadenas siguen el escapado producido por
`serde_json`; los dos enteros se representan como números decimales JSON sin
comillas.

Las versiones de referencia quedan fijadas en:

- `serde 1.0.229`
- `serde_json 1.0.151`

El veredicto de la auditoría para esta implementación y este dominio de entrada
es `NO AMBIGUO`: las claves, la estructura JSON y el escapado separan los
campos, por lo que no se aplica la ambigüedad propia de una concatenación
desnuda.

## Evidencia ejecutable

Los siete vectores congelados se encuentran en:

```text
tests/fixtures/canonical_vectors.json
```

El test que llama a la función real de producción y compara los resultados byte
a byte se encuentra en:

```text
tests/canonical_vectors.rs
```

Los vectores cubren valores mínimos, comillas, contenido con apariencia de
estructura JSON, intercambio de valores entre campos, `u64::MAX`,
determinismo entre instancias separadas y texto Unicode no ASCII.

## Requisitos para G3

`protocol_core` debe reproducir el mismo struct de siete campos, con idénticos
nombres, orden y tipos. No debe modelarlo como una abstracción genérica
`payload + context + nonce`.

G3 debe reutilizar las versiones fijadas de `serde` y `serde_json` en lugar de
construir manualmente el JSON. Una serialización manual podría generar JSON
semánticamente equivalente pero bytes diferentes.

En Rust `no_std`, el uso de `serde_json::to_vec` requiere `alloc` y un allocator
embebido. El primer spike sobre el target ARM debe confirmar:

1. que las versiones fijadas compilan con la configuración `no_std + alloc`;
2. que reproduce los siete fixtures byte a byte;
3. el consumo conjunto de flash, heap, RAM y stack de la serialización y
   ML-DSA-65;
4. el margen restante en el LPC55S69 bajo la configuración prevista.

## Límites y pendientes

El campo `context` pertenece al modelo de aplicación y se serializa dentro del
JSON. No utiliza el parámetro contextual nativo de ML-DSA.

El formato no contiene un campo explícito de dominio o versión. El hallazgo se
mantiene como pendiente no bloqueante y no se resuelve en esta decisión para
evitar modificar un formato ya verificado.

Cualquier modificación futura del nombre, tipo, orden o representación de los
campos se tratará como un cambio de protocolo y deberá actualizar esta decisión
y sus vectores de compatibilidad.