pub fn arrays_intersection(arr1: Vec<i32>, arr2: Vec<i32>, arr3: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut result = Vec::new();

    while i < arr1.len() && j < arr2.len() && k < arr3.len() {
        if arr1[i] == arr2[j] && arr2[j] == arr3[k] {
            result.push(arr1[i]);
            i += 1;
            j += 1;
            k += 1;
        } else {
            let i2 = i;
            let j2 = j;
            let k2 = k;
            if arr1[i2] < arr2[j2] || arr1[i2] < arr3[k2] {
                i += 1;
            }
            if arr2[j2] < arr1[i2] || arr2[j2] < arr3[k2] {
                j += 1;
            }
            if arr3[k2] < arr1[i2] || arr3[k2] < arr2[j2] {
                k += 1;
            }
        }

    }
    result
}
