#[cfg(test)]
#[test]
fn check_ratio() {
    let width = 1600;
    let height = 500;

    let nwidth = 500;

    fn ratio(width: u32, height: u32) -> f32 {
        let ratio = height as f32 / width as f32;
        ratio.round()
    }

    fn resize(nwidth: u32, width: u32, height: u32) -> u32 {
        return height * nwidth / width;
    }

    let before_ration = ratio(width, height);
    let after_ration = ratio(nwidth, resize(nwidth, width, height));

    assert_eq!(before_ration, after_ration);
}
