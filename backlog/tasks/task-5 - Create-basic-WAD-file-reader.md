---
id: task-5
title: Create basic WAD file reader
status: To Do
assignee: []
created_date: '2025-07-09'
updated_date: '2025-07-09'
labels: []
dependencies: []
---

## Description

Build a simple WAD file reader that can list the contents. Learn about file format parsing and data validation.

## Acceptance Criteria

- [ ] Opens a WAD file successfully
- [ ] Reads the header (signature num_lumps directory_offset)
- [ ] Lists all lump names in the file
- [ ] Validates the WAD signature is correct

## Implementation Plan

1. Learn about the WAD file format (header + directory structure)
2. Use std::fs::File to open the WAD file
3. Read the 12-byte header (signature, numlumps, diroffset)
4. Seek to directory and read lump entries
5. Display list of all lumps in the WAD file
