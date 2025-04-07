fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::usize;

    #[test]
    fn trap_water() {
        // 双向指针：
        // min{lmax,rmax}-h[i]
        // lmax 指向 当前节点左边最大高度
        // rmax 指向 当前节点右边最大高度
        // let height = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        // let mut left_max = 0;
        // let mut right_max = 0;
        // let mut min = 0;
        // let mut sum = 0;
        // height.iter().enumerate().for_each(|(idx, &item)| {
        //     // 移动left_max
        //     left_max = *height[..=idx].iter().max().unwrap_or(&0);
        //     right_max = *height[idx..].iter().max().unwrap_or(&0);
        //     min = *[left_max, right_max].iter().min().unwrap();
        //     sum += min - item;
        // });
        // println!("{sum}");
        let height = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        height
            .iter()
            .enumerate()
            .fold(0i32, |mut sum, (idx, &item)| {
                let lm = *height[..=idx].iter().max().unwrap_or(&0);
                let rm = *height[idx..].iter().max().unwrap_or(&0);
                let min = *[lm, rm].iter().min().unwrap();
                sum += min - item;
                return sum;
            });
    }
}
