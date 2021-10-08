// Courtesy of https://www.kvraudio.com/forum/viewtopic.php?t=291758

pub fn parameter_to_decibels(param: f32) -> f32 {
    // FIXME: I'm totally guessing on this implementation!

    18.0 * param.log(1.5)

    // 8.3, 1.2: off by ~11.1
    // 18, 1.5: off by ~10.1
    // 21, 1.6: off by ~10.3

    // 24, 1.7, off by ~10.8

    // 26.4, 1.8: off by ~10.5

    // 29, 1.9: off by ~10.8

    // 31, 2.0: off by ~10.4
}

pub fn amplitude_to_decibels(amplitude: f32) -> f32 {
    // FIXME: implement
    amplitude
}

pub fn decibels_to_amplitude(db: f32) -> f32 {
    (10.0_f32).powf(db * 0.05) // 10^(dB/20)


    // https://doc.rust-lang.org/std/primitive.f64.html#method.exp
    // Involves Euler's number
    // return exp(dB * 0.115129254649702195134608473381376825273036956787109375);
}

// inline double amp2dB(const double amp)
// {
//     // input must be positive +1.0 = 0dB
//     if (amp < 0.0000000001) { return -200.0; }
//     return (20.0 * log10(amp));
// }
// inline double dB2amp(const double dB)
// {
//   // 0dB = 1.0
//   //return pow(10.0,(dB * 0.05)); // 10^(dB/20)
//   return exp(dB * 0.115129254649702195134608473381376825273036956787109375);
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_maximum() {
        assert_eq!(parameter_to_decibels(1.0), 0.0);
    }

    // #[test]
    // fn test_param_minus_6() {
    //     assert_eq!(parameter_to_decibels(0.875), -6.0);
    // }

    // #[test]
    // fn test_param_minus_12() {
    //     assert_eq!(parameter_to_decibels(0.7586), -12.0);
    // }

    // #[test]
    // fn test_param_minus_30() {
    //     assert_eq!(parameter_to_decibels(0.4052), -30.0);
    // }

    // #[test]
    // fn test_param_near_zero() {
    //     assert_eq!(parameter_to_decibels(0.001), -60.0);
    // }

    #[test]
    fn test_decibel_zero() {
        assert_eq!(decibels_to_amplitude(0.0), 1.0);
    }

    #[test]
    fn test_decibel_minus_6() {
        assert_eq!(decibels_to_amplitude(-6.0), 0.5);
    }
}
