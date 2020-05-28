use crate::{ImgSize, ImgBuf, Img, ImgMut};

/// Scale image to different resolution by nearest neighbor
///
/// # Arguments
/// * `image` - input image 
/// * `size` - target image size
#[inline(never)]
pub fn resize_nearest_new<T: Copy + Default>(
    image: &dyn Img<T>,
    size: ImgSize
) -> ImgBuf<T> {
    let x_indices = scale_index_table(image.width(), size.x);
    let y_indices = scale_index_table(image.height(), size.y);
    let mut result = ImgBuf::<T>::new_init(size, Default::default());

    for line in 0..size.y {
        let dst = result.line_mut(line);
        let src = image.line_ref(y_indices[line]);

        for x in 0..size.x {
            dst[x] = src[x_indices[x]];
        }
    }

    result
}

/// Generates lookup table for scaling source vector info target vector
/// of different size
fn scale_index_table(source_size: usize, target_size: usize) -> Vec<usize> {
    (0..target_size)
        .map(|x| x*source_size/target_size)
        .collect()
}

// ================================== TESTS ==================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_size_6_into_3() {
        assert_eq!(scale_index_table(6, 3), vec![0, 2, 4]);       
    }


    #[test]
    fn scale_size_3_into_6() {
        assert_eq!(scale_index_table(3, 6), vec![0, 0, 1, 1, 2, 2]);
    }
    
    
    #[test]
    fn scale_size_2_into_4() {
        assert_eq!(scale_index_table(2, 4), vec![0, 0, 1, 1]);
    }

    #[test]
    fn scale_2x2_to_4x4() {
        assert_eq!(
            resize_nearest_new(
                &ImgBuf::from_vec(ImgSize::new(2, 2), vec![1, 2, 3, 4]), 
                ImgSize::new(4, 4)
            ),
            ImgBuf::from_vec(ImgSize::new(4, 4), vec![
                1, 1, 2, 2,
                1, 1, 2, 2,
                3, 3, 4, 4,
                3, 3, 4, 4,
            ]), 
        )
    }
}