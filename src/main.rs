use std;

fn quickSort<E: Ord>(slice: &mut [E])
{
    //If length == 1; return
    //If length == 2; sort and return
    //Take first element as pivot, split left / right slices, shift elements
    //quickSort(left); quickSort(right);
}

fn main() 
{

}

#[cfg(tests)]
mod tests
{
    use super::*;

    #[test]
    fn increasing()
    {
        assert_eq!([1,2,3,4,5], quicksort([1,2,3,4,5]));
    }

    #[test]
    fn decreasing()
    {
        assert_eq!([1,2,3,4,5], quicksort([5,4,3,2,1]));
    }

    #[test]
    fn identity()
    {
        assert_eq!([1,1,1,1,1], quicksort([1,1,1,1,1]));
    }

    #[test]
    fn negatives()
    {
        assert_eq!([-1,-1,1,1,1], quicksort([1,-1,-1,1,1]));
    }

    #[test]
    fn empty()
    {
        assert_eq!([], quicksort([]));
    }

    #[test]
    fn single()
    {
        assert_eq!([1], quicksort([1]));
    }

    #[test]
    fn scattered()
    {
        assert_eq!([1,2,3,4,5], quicksort([3,1,5,4,2]));
    }
}

