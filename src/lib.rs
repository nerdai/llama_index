use pyo3::prelude::*;

#[pyfunction]
fn add(left: usize, right: usize) -> usize {
    left + right
}

fn _close_chunk(
    chunks: &mut Vec<String>,
    cur_chunk: Vec<(String, usize)>,
    chunk_overlap: usize,
) -> (Vec<(String, usize)>, usize) {
    let cur_chunk_string: String = cur_chunk
        .iter()
        .map(|(x, _)| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    chunks.push(cur_chunk_string);
    let last_chunk: Vec<(String, usize)> = cur_chunk;
    let mut cur_chunk = vec![];
    let mut cur_chunk_len = 0;

    if last_chunk.len() > 0__usize {
        let mut last_index = (last_chunk.len() - 1) as i32;
        while (last_index >= 0) && (cur_chunk_len + last_chunk.last().unwrap().1 <= chunk_overlap) {
            let (text, length) = last_chunk.last().unwrap();
            cur_chunk_len += length;
            cur_chunk.insert(0, (text.to_string(), *length));
            last_index -= 1;
        }
    }
    return (cur_chunk, cur_chunk_len);
}

#[pyfunction]
fn _merge_splits(mut reversed_splits: Vec<(&str, bool, usize)>, chunk_size: usize, chunk_overlap: usize) -> Vec<String> {
    let mut chunks: Vec<String> = vec![];
    let mut cur_chunk: Vec<(String, usize)> = vec![];
    let mut cur_chunk_len: usize = 0;
    let mut new_chunk: bool = true;

    while reversed_splits.len() > 0 {
        let cur_split = reversed_splits.last().unwrap();
        let (text, is_sentence, token_size) = cur_split;

        if cur_chunk_len + token_size > chunk_size && !new_chunk {
            (cur_chunk, cur_chunk_len) = _close_chunk(
                &mut chunks,
                cur_chunk,
                chunk_overlap,
            );
            new_chunk = true;
        } else {
            if *is_sentence || (cur_chunk_len + token_size <= chunk_size) || (new_chunk) {
                cur_chunk_len += token_size;
                cur_chunk.push((String::from(*text), *token_size));
                reversed_splits.pop();
                new_chunk = false;
            } else {
                (cur_chunk, cur_chunk_len) = _close_chunk(
                    &mut chunks,
                    cur_chunk,
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
