# Birf

WIP thing that tries to find xxh3_64 hash collisions.

Made it as far as 1879048225 hashes before my computer exploded.

# TODO

- replace in-memory hashtable with a fixed-size on-disk hashtable. Memory is not large enough so we need SSD.
  - resume support?
  - rehash support? (lets you grow the table)
  - investigate Write vs Memmap. Memmap is probably correct but who knows man
