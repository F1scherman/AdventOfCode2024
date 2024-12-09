fn main() {
    let input = include_str!("input.txt");
    part_two(input);
}

#[derive(Clone, Copy)]
enum DiskEntry {
    Free(),
    Allocated(usize),
}

fn part_one(input: &str) {
    let mut disk = vec![];
    let mut id_counter = 0;
    let mut free = false;
    for block in input.chars().map(|x| x.to_string().parse::<i32>().unwrap()) {
        if free {
            free = false;
            for _ in 0..block {
                disk.push(DiskEntry::Free());
            }
        } else {
            free = true;
            for _ in 0..block {
                disk.push(DiskEntry::Allocated(id_counter));
            }
            id_counter += 1;
        }
    }

    let mut left_pointer = 0;
    let mut right_pointer = disk.len() - 1;
    while left_pointer < right_pointer {
        while let DiskEntry::Allocated(_) = disk[left_pointer] {
            left_pointer += 1;
        }
        while let DiskEntry::Free() = disk[right_pointer] {
            right_pointer -= 1;
        }
        if left_pointer < right_pointer {
            let temp = disk[left_pointer];
            disk[left_pointer] = disk[right_pointer];
            disk[right_pointer] = temp;
        }
    }

    let mut sum = 0;
    for i in 0..disk.len() {
        if let DiskEntry::Allocated(id) = disk[i] {
            sum += id * i;
        }
    }

    println!("{}", sum);
}

fn part_two(input: &str) {
    let mut disk = vec![];
    let mut id_counter = 0;
    let mut free = false;
    for block in input.chars().map(|x| x.to_string().parse::<i32>().unwrap()) {
        if free {
            free = false;
            for _ in 0..block {
                disk.push(DiskEntry::Free());
            }
        } else {
            free = true;
            for _ in 0..block {
                disk.push(DiskEntry::Allocated(id_counter));
            }
            id_counter += 1;
        }
    }

    for id in (0..id_counter).rev() {
        let old_alloc_block_start = disk
            .iter()
            .position(|x| match x {
                DiskEntry::Free() => false,
                DiskEntry::Allocated(id2) => *id2 == id,
            })
            .unwrap_or(disk.len());
        if old_alloc_block_start >= disk.len() {
            continue;
        }
        let mut old_alloc_block_end = disk[old_alloc_block_start..]
            .iter()
            .position(|x| match x {
                DiskEntry::Free() => true,
                DiskEntry::Allocated(id2) => *id2 != id,
            })
            .unwrap_or(disk.len() - 1);
        if old_alloc_block_end != disk.len() - 1 {
            old_alloc_block_end += old_alloc_block_start;
            old_alloc_block_end -= 1;
        }
        // println!("Id: {} from {} to {}", id, old_alloc_block_start, old_alloc_block_end);
        let mut free_block_start = disk.len();
        let mut size_counter = 0;
        let mut counting = false;
        for i in 0..disk.len() {
            if let DiskEntry::Free() = disk[i] {
                if counting {
                    size_counter += 1;
                } else {
                    counting = true;
                    size_counter = 1;
                }
            } else {
                if counting {
                    counting = false;
                    if size_counter >= (old_alloc_block_end - old_alloc_block_start + 1) {
                        free_block_start = i - size_counter;
                        break;
                    } else {
                        size_counter = 0;
                    }
                }
            }
        }

        if free_block_start > old_alloc_block_start {
            continue;
        }
        // println!("Moved {}: {} to {}", id, free_block_start, old_alloc_block_end - old_alloc_block_start + free_block_start);
        for i in old_alloc_block_start..=old_alloc_block_end {
            disk[i] = DiskEntry::Free();
        }

        for i in free_block_start..=(old_alloc_block_end - old_alloc_block_start + free_block_start)
        {
            disk[i] = DiskEntry::Allocated(id);
        }
    }

    let mut sum = 0;
    for i in 0..disk.len() {
        if let DiskEntry::Allocated(id) = disk[i] {
            sum += id * i;
        }
    }

    println!("{}", sum);
}
