#[derive(Debug, Eq, PartialEq)]
pub struct Solution {}

impl Solution {
  pub fn three_sum_smaller(numbers: &mut Vec<isize>, target: isize) -> usize {
    /*
      - ok since we are not asking for i,j,k specifically
        just the total number of triplets, it's fine we
        sort the input and potentially change the index
        of the numbers - the result will be the same
      - here is an example: let say numbers = [0,1,3,-2]
        without sorting the numbers array the two triplets
        are: [0,1,-2] and [0, 3, 2]. The total number of
        triplets is still 2.
      - so why we sort the numbers array then? If it will
        save time in each run trying to find a triplets
    */
    numbers.sort();
    let total_numbers = numbers.len();
    let mut no_triplets = 0;

    for index in 0..total_numbers - 2 {
      /*
        - for each index scan through the
          range of indexes from index + 1
          to total_numbers - 1;
      */
      let mut lo = index + 1;
      let mut hi = total_numbers - 1;

      while lo < hi {
        if numbers[index] + numbers[lo] + numbers[hi] < target {
          /*
            - we sorted the numbers array
            - numbers[hi] is the max number
              in the range from lo to hi
            - so you can assume everything
              comes before numbers numbers[hi]
              will aslo work as they are smaller
              such as numbers[hi-1]
          */
          no_triplets += hi - lo;
          lo += 1;
        } else {
          hi -= 1;
        }
      }
    }

    no_triplets
  }
}

pub struct TestFixtures {}
impl TestFixtures {
  pub fn test_fixture_1() -> Vec<isize> {
    vec![-2, 0, 1, 3]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let result = Solution::three_sum_smaller(&mut TestFixtures::test_fixture_1(), 2);
    assert_eq!(result, 2);
  }
}
