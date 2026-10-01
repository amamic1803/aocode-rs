use aocode::{AdventOfCode, AoC as libAoC};
use js_sys::{Array, Number, Object, Reflect};
use pmath::statistics::Sample as libSample;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AoC {
    inner: libAoC,
}
#[wasm_bindgen]
impl AoC {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: libAoC::new(),
        }
    }

    pub fn years(&self) -> Result<Array, JsValue> {
        let arr = Array::new();
        for y in self.inner.years() {
            let obj = Number::from(y.id() as u32);
            arr.push(&obj);
        }
        Ok(arr)
    }

    pub fn days(&self, year: usize) -> Result<Array, JsValue> {
        let arr = Array::new();
        for d in self.inner.year(year).unwrap().days() {
            let obj = Object::new();
            Reflect::set(&obj, &JsValue::from_str("id"), &Number::from(d.id() as u32))?;
            Reflect::set(
                &obj,
                &JsValue::from_str("title"),
                &JsValue::from_str(d.title()),
            )?;
            arr.push(&obj);
        }
        Ok(arr)
    }

    pub fn solve(
        &self,
        year: usize,
        day: usize,
        part: usize,
        input: &str,
    ) -> Result<String, JsValue> {
        self.inner
            .solve(year, day, part, input)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn benchmark(
        &self,
        year: usize,
        day: usize,
        part: usize,
        input: &str,
    ) -> Result<Object, JsValue> {
        let (res, dur) = self
            .inner
            .benchmark(year, day, part, input)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let obj = Object::new();
        Reflect::set(&obj, &JsValue::from_str("result"), &JsValue::from_str(&res))?;
        Reflect::set(
            &obj,
            &JsValue::from_str("duration"),
            &Number::from(dur.as_nanos() as f64),
        )?;

        Ok(obj)
    }
}
impl Default for AoC {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub struct Sample {
    inner: libSample<f64>,
}
#[wasm_bindgen]
impl Sample {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: libSample::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn push(&mut self, value: f64) {
        self.inner.push(value);
    }

    pub fn mean(&self) -> Option<f64> {
        self.inner.mean()
    }

    pub fn stddev(&self) -> Option<f64> {
        self.inner.stddev()
    }
}
impl Default for Sample {
    fn default() -> Self {
        Self::new()
    }
}
