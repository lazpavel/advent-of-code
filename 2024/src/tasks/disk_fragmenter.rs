use crate::tasks::utils::read_file_to_digits;

#[derive(Clone, PartialEq, Debug)]
enum DiskDataType {
  SPACE,
  FILE,
}

#[derive(PartialEq, Debug, Clone)]
struct DiskData {
  pub entries: Vec<DiskDataEntry>,
}

#[derive(Clone, PartialEq, Debug)]
struct DiskDataEntry {
  pub id: Option<usize>,
  pub data_type: DiskDataType,
  pub size: usize,
}

pub fn run() -> (usize, usize) {
  let input = read_file_to_digits("./inputs/disk_fragmenter.txt").unwrap();
  let mut split_file_data = build_disk_data(&input);
  let whole_file_data = split_file_data.clone();

  compress_data(&mut split_file_data);
  let checksum = calculate_checksum(split_file_data);

  let compressed = compress_v2(whole_file_data);
  let compressed_checksum = calculate_checksum(compressed);

  (checksum, compressed_checksum)
}

fn compress_v2(uncompressed: DiskData) -> DiskData {
  let mut compressed = DiskData { entries: vec![] };

  let mut uncompressed_entries = uncompressed.entries.clone();
  for i in 0..uncompressed_entries.len() {
    let entry = &uncompressed_entries[i];
    if entry.data_type == DiskDataType::FILE {
      compressed.entries.push(entry.clone());
    } else {
      let (fill_entries, left, indexes) =
        get_fill_entries(&uncompressed_entries, i + 1, entry.size);
      for f in fill_entries {
        compressed.entries.push(f);
      }

      if left > 0 {
        compressed.entries.push(DiskDataEntry {
          id: None,
          data_type: DiskDataType::SPACE,
          size: left,
        })
      }

      // Separate scope for mutable borrow
      {
        for &j in &indexes {
          uncompressed_entries[j] = DiskDataEntry {
            data_type: DiskDataType::SPACE,
            id: None,
            size: uncompressed_entries[j].size,
          };
        }
      }
    }
  }

  compressed
}

fn get_fill_entries(
  entries: &Vec<DiskDataEntry>,
  i: usize,
  size: usize,
) -> (Vec<DiskDataEntry>, usize, Vec<usize>) {
  let mut extracted_files = Vec::new();
  let mut remaining_size = size;
  let mut indexes = vec![];

  for i in (i..entries.len()).rev() {
    if entries[i].size <= remaining_size && entries[i].data_type == DiskDataType::FILE {
      remaining_size -= entries[i].size;
      extracted_files.push(entries[i].clone());
      indexes.push(i);
    }
  }

  (extracted_files, remaining_size, indexes)
}

fn calculate_checksum(disk_data: DiskData) -> usize {
  let mut checksum = 0;
  let mut position = 0;
  for data in disk_data.entries {
    if data.data_type == DiskDataType::SPACE {
      position += data.size;
      continue;
    }

    for i in 0..data.size {
      checksum += (position + i) * data.id.unwrap();
    }

    position += data.size;
  }

  checksum
}

fn compress_data(disk_data: &mut DiskData) {
  loop {
    let mut gap_found = false;
    let mut index = 0;
    for data in &disk_data.entries {
      if data.data_type == DiskDataType::SPACE && index < disk_data.entries.len() - 1 {
        gap_found = true;
        break;
      }

      index += 1;
    }

    if !gap_found {
      break;
    }

    let (entries, _removed) =
      get_end_file_entries(index, disk_data.entries[index].size, &mut disk_data.entries);

    if index < disk_data.entries.len() {
      disk_data.entries.remove(index);
    }

    for (i, element) in entries.into_iter().enumerate() {
      disk_data.entries.insert(index + i, element);
    }
  }
}

fn get_end_file_entries(
  index: usize,
  size: usize,
  entries: &mut Vec<DiskDataEntry>,
) -> (Vec<DiskDataEntry>, usize) {
  let mut extracted_files = Vec::new();
  let mut remaining_size = size;
  let mut removed = 0;

  while remaining_size > 0 && index < entries.len() {
    if let Some(last_entry) = entries.pop() {
      removed += 1;

      if last_entry.data_type == DiskDataType::FILE {
        if last_entry.size <= remaining_size {
          remaining_size -= last_entry.size;
          extracted_files.push(last_entry);
        } else {
          // If the last entry is larger than the remaining size, split it
          let mut remaining_entry = last_entry.clone();
          remaining_entry.size -= remaining_size;
          removed -= 1;
          entries.push(remaining_entry);

          let mut extracted_entry = last_entry;
          extracted_entry.size = remaining_size;
          extracted_files.push(extracted_entry);

          remaining_size = 0;
        }
      }
    } else {
      break;
    }
  }

  (extracted_files, removed)
}

fn build_disk_data(input: &[u8]) -> DiskData {
  let mut disk_data = DiskData { entries: vec![] };

  let mut index = 0;
  let mut id = 0;
  for data in input {
    if index % 2 == 0 {
      disk_data.entries.push(DiskDataEntry {
        id: Some(id),
        size: (*data).into(),
        data_type: DiskDataType::FILE,
      });

      id += 1;
    } else {
      disk_data.entries.push(DiskDataEntry {
        id: None,
        data_type: DiskDataType::SPACE,
        size: (*data).into(),
      })
    }

    index += 1
  }

  disk_data
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_run() {
    assert_eq!(run(), (6332189866718, 6353648390778));
  }

  #[test]
  fn test_build_data() {
    let input = vec![1, 2, 3, 4, 5];
    let disk_data = build_disk_data(&input);

    assert_eq!(disk_data.entries.len(), 5);
  }

  #[test]
  fn test_checksum() {
    assert_eq!(
      calculate_checksum(DiskData {
        entries: vec![
          DiskDataEntry {
            id: Some(0),
            data_type: DiskDataType::FILE,
            size: 2,
          },
          DiskDataEntry {
            id: Some(9),
            data_type: DiskDataType::FILE,
            size: 2,
          },
          DiskDataEntry {
            id: Some(2),
            data_type: DiskDataType::FILE,
            size: 1,
          },
          DiskDataEntry {
            id: Some(1),
            data_type: DiskDataType::FILE,
            size: 3,
          },
          DiskDataEntry {
            id: Some(7),
            data_type: DiskDataType::FILE,
            size: 3,
          },
          DiskDataEntry {
            id: None,
            data_type: DiskDataType::SPACE,
            size: 1,
          },
          DiskDataEntry {
            id: Some(4),
            data_type: DiskDataType::FILE,
            size: 2,
          },
          DiskDataEntry {
            id: None,
            data_type: DiskDataType::SPACE,
            size: 1,
          },
          DiskDataEntry {
            id: Some(3),
            data_type: DiskDataType::FILE,
            size: 3,
          },
          DiskDataEntry {
            id: None,
            data_type: DiskDataType::SPACE,
            size: 1,
          },
          DiskDataEntry {
            id: None,
            data_type: DiskDataType::SPACE,
            size: 2,
          },
          DiskDataEntry {
            id: None,
            data_type: DiskDataType::SPACE,
            size: 1,
          },
          DiskDataEntry {
            id: Some(5),
            data_type: DiskDataType::FILE,
            size: 4,
          },
          DiskDataEntry {
            id: None,
            data_type: DiskDataType::SPACE,
            size: 1,
          },
          DiskDataEntry {
            id: Some(6),
            data_type: DiskDataType::FILE,
            size: 4,
          },
          DiskDataEntry {
            id: None,
            data_type: DiskDataType::SPACE,
            size: 1,
          },
          DiskDataEntry {
            id: None,
            data_type: DiskDataType::SPACE,
            size: 3,
          },
          DiskDataEntry {
            id: None,
            data_type: DiskDataType::SPACE,
            size: 1,
          },
          DiskDataEntry {
            id: Some(8),
            data_type: DiskDataType::FILE,
            size: 4,
          },
          DiskDataEntry {
            id: None,
            data_type: DiskDataType::SPACE,
            size: 2,
          }
        ]
      }),
      2858
    );
  }
}
