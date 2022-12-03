
pub mod task1;
pub mod task2;

type Points = u32;
fn sum_of_points(iter: impl Iterator<Item = char>) -> Points {
    let priorities_sum = iter
    .map(|c| {
        let mut buffer: [u8; 1] = [0; 1];
        c.encode_utf8(&mut buffer);
        let priority = if buffer[0] < 97 {
            buffer[0] - 38
        } else {
            buffer[0] - 96
        };
        priority as u32
    })
    .sum::<u32>();
    priorities_sum
}