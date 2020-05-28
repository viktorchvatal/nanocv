use crate::{Img, ImgMut, ImgBuf};

pub fn mirror_horizontal_new<T: Copy + Default>(input: &dyn Img<T>) -> ImgBuf<T> {
    let mut output = ImgBuf::new(input.size());

    for line in 0..input.height() {
        let src = input.line_ref(line);
        let dst = output.line_mut(line);
        let last = input.width() - 1;

        for column in 0..input.width() {
            dst[column] = src[last - column];
        }
    }

    output
}


pub fn mirror_vertical_new<T: Copy + Default>(input: &dyn Img<T>) -> ImgBuf<T> {
    let mut output = ImgBuf::new(input.size());

    for line in 0..input.height() {
        let src = input.line_ref(line);
        let dst = output.line_mut(input.height() - 1 - line);        
        dst.clone_from_slice(src);
    }

    output
}

// ================================== TESTS ==================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ImgSize;

    #[test]
    fn test_mirror_horizontal() {
        assert_eq!(
            mirror_horizontal_new(
                &ImgBuf::from_vec(
                    ImgSize::new(3, 3),
                    vec![
                        1, 2, 3,
                        4, 5, 6,
                        7, 8, 9,
                    ]
                )
            ),
            ImgBuf::from_vec(
                ImgSize::new(3, 3),
                vec![
                    3, 2, 1, 
                    6, 5, 4, 
                    9, 8, 7, 
                ]
            )            
        )
    }


    #[test]
    fn test_mirror_vertical() {
        assert_eq!(
            mirror_vertical_new(
                &ImgBuf::from_vec(
                    ImgSize::new(3, 3),
                    vec![
                        1, 2, 3,
                        4, 5, 6,
                        7, 8, 9,
                    ]
                )
            ),
            ImgBuf::from_vec(
                ImgSize::new(3, 3),
                vec![
                    7, 8, 9,
                    4, 5, 6,
                    1, 2, 3,
            ]
            )            
        )
    }    
}