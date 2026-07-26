use super::*;

// Оба сабмодуля намеренно экспортируют одноимённую `generate` — это две
// реализации одного контракта, вызываемые квалифицированно
// (`empty::generate()` / `perlin_noise::generate()`). Не переименовывать.
pub mod empty;
pub mod perlin_noise;
