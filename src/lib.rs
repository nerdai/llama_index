use pyo3::prelude::*;
use std::{collections::VecDeque, slice::Chunks};

#[pyfunction]
fn add(left: usize, right: usize) -> usize {
    left + right
}

fn _close_chunk(
    chunks: &mut Vec<String>,
    cur_chunk: &mut Vec<(String, usize)>,
    mut cur_chunk_len: usize,
    last_chunk: &mut Vec<(String, usize)>,
    mut new_chunk: bool,
    chunk_overlap: &mut usize,
) {
    let cur_chunk_string: String = cur_chunk
        .iter()
        .map(|(x, _)| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    chunks.push(cur_chunk_string);
    cur_chunk = &mut Vec::new();
    cur_chunk_len = 0__usize;
    new_chunk = true;

    if last_chunk.len() > 0__usize {
        let mut last_index = (last_chunk.len() - 1) as i32;
        while (last_index >= 0) && (cur_chunk_len + last_chunk.last().unwrap().1 <= chunk_overlap) {
            let (text, length) = last_chunk.last().unwrap();
            cur_chunk_len += length;
            cur_chunk.insert(0, (text.to_string(), *length));
            last_index -= 1;
        }
    }
}

#[pyfunction]
fn _merge_splits(mut splits: VecDeque<(&str, bool, usize)>, chunk_size: usize) -> Vec<String> {
    let mut new_chunk: bool = true;
    let mut chunks: Vec<String> = vec![];
    while splits.len() > 0 {
        let cur_split = splits[0];
        let (text, is_sentence, token_size) = cur_split;

        if token_size > chunk_size && !new_chunk {
            _close_chunk(
                chunks,
                cur_chunk,
                cur_chunk_len,
                last_chunk,
                new_chunk,
                chunk_overlap,
            );
        } else {
            if is_sentence || (cur_chunk_len + token_size <= chunk_size) || (new_chunk) {
                cur_chunk_len += token_size;
                cur_chunk.push((text, token_size));
                splits.pop_front();
                new_chunk = false;
            } else {
                _close_chunk(
                    chunks,
                    cur_chunk,
                    cur_chunk_len,
                    last_chunk,
                    new_chunk,
                    chunk_overlap,
                );
            }
        }
    }

    // handle the last chunk
    if !new_chunk {
        let cur_chunk_string: String = cur_chunk
            .iter()
            .map(|(x, _)| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        chunks.push(cur_chunk_string);
    }

    return chunks;
}

#[pymodule]
#[pyo3(name = "_rust")]
fn module_extension(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(_merge_splits, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
