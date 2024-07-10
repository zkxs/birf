Made it as far as `1879048225`

# TODO

- replace in-memory hashtable with a fixed-size on-disk hashtable. Memory is not large enough so we need SSD.
  - resume support
  - rehash support (lets you grow the table)
  - investigate Write vs Memmap
