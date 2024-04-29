use crate::model::observables::Magnetization;

#[derive(Debug, PartialEq)]
struct MeanMagnetization {
    value: f64,
    uncertainty: f64,
}
fn sample_mean_magnetization<'a>(
    samples: impl Iterator<Item = &'a Magnetization>,
) -> Option<MeanMagnetization> {
    let (count, sum, sum_squared) = samples.fold((0, 0, 0), |(count, sum, sum_squared), m| {
        (count + 1, sum + m.0, sum_squared + m.0 * m.0)
    });
    if count == 0 {
        return None;
    }
    if count == 1 {
        return Some(MeanMagnetization {
            value: sum as f64,
            uncertainty: 0.0,
        });
    }
    let mean = sum as f64 / count as f64;
    let squared_mean = sum_squared as f64 / count as f64;
    Some(MeanMagnetization {
        value: mean,
        uncertainty: (count as f64 / (count as f64 - 1.) * (squared_mean - mean * mean)).sqrt(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_of_nothing_is_nothing() {
        assert_eq!(sample_mean_magnetization(vec![].iter()), None);
    }

    #[test]
    fn one_item_gives_exact_mean() {
        assert_eq!(
            sample_mean_magnetization(vec![Magnetization(1)].iter()),
            Some(MeanMagnetization {
                value: 1.0,
                uncertainty: 0.0
            })
        );
    }

    #[test]
    fn several_same_items_give_exact_mean() {
        assert_eq!(
            sample_mean_magnetization(
                vec![Magnetization(4), Magnetization(4), Magnetization(4)].iter()
            ),
            Some(MeanMagnetization {
                value: 4.0,
                uncertainty: 0.0
            })
        );
    }

    #[test]
    fn several_different_items_give_uncertainty() {
        assert_eq!(
            sample_mean_magnetization(
                vec![
                    Magnetization(2),
                    Magnetization(4),
                    Magnetization(4),
                    Magnetization(4),
                    Magnetization(5),
                    Magnetization(5),
                    Magnetization(7),
                    Magnetization(9)
                ]
                .iter()
            ),
            Some(MeanMagnetization {
                value: 5.0,
                uncertainty: (32. as f64 / 7.).sqrt()
            })
        );
    }

    #[test]
    fn values_around_zero_give_uncertainty() {
        assert_eq!(
            sample_mean_magnetization(
                vec![Magnetization(-2), Magnetization(2), Magnetization(0)].iter()
            ),
            Some(MeanMagnetization {
                value: 0.0,
                uncertainty: 2.0
            })
        )
    }
}
