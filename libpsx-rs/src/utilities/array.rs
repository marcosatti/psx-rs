use crate::types::geometry::*;

/// Flip rows about the centre and returns the processed data in a new array.
/// Does not touch ordering within rows. Array size must be divisible by row_length, panics otherwise.
///
/// Example:
/// y1 y2 y3           x1 x2 x3
/// ...         -->    ...
/// x1 x2 x3           y1 y2 y3
pub(crate) fn flip_rows<T: Clone>(array: &[T], row_length: usize) -> Vec<T> {
    let total_count = array.len();
    let total_row_count = total_count / row_length;
    assert!(total_row_count * row_length == total_count);

    let mut new_array = Vec::new();
    new_array.reserve(total_count);

    let mut row_count = 0;
    while row_count < total_row_count {
        let base_index = (total_row_count - row_count - 1) * row_length;
        let slice = &array[base_index..(base_index + row_length)];
        new_array.extend_from_slice(slice);
        row_count += 1;
    }

    new_array
}

/// Extracts a rectangular partition out of an array, by assuming a row is made up of size.width contiguous elements,
/// with size.height contiguous rows. Origin must be at the lower left corner.
///
/// Example with origin = (1, 1) and size = (3, 2):
///                     Max index
///           - - - - -
///           - e e e -
///           - e e e -
///           - - - - -
/// Min index
/// The items marked as 'e' are extracted into a new flattened array.
pub(crate) fn extract_rectangle<T: Clone>(array: &[T], row_length: usize, origin: Point2D<usize, UnknownUnit>, size: Size2D<usize, UnknownUnit>) -> Vec<T> {
    let mut rect_buffer = Vec::new();
    rect_buffer.reserve(size.width * size.height);

    let mut row_count = 0;
    while row_count < size.height {
        let base_index = (origin.y + row_count) * row_length + origin.x;
        let slice = &array[base_index..(base_index + size.width)];
        rect_buffer.extend_from_slice(slice);
        row_count += 1;
    }

    rect_buffer
}
