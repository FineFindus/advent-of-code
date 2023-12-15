/// Find the largest element in the given array.
/// Returns 0 if the array is empty.
int
arr_max_element (int *arr, int len)
{
  int largest = 0;
  for (int i = 0; i < len; i++)
    {
      if (arr[i] > largest)
        largest = arr[i];
    }
  return largest;
}

/// Returns the index of the element in the array.
/// -1 is returned if the element is not found.
int
arr_find_index (int *arr, int len, int value)
{
  for (int i = 0; i < len; i++)
    {
      if (arr[i] == value)
        {
          return i;
        }
    }
  return -1;
}

void
arr_reverse (int *arr, int len)
{
  for (int i = 0; i < len / 2; i++)
    {
      int temp = arr[i];
      arr[i] = arr[len - 1 - i];
      arr[len - 1 - i] = temp;
    }
}
