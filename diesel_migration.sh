~/.cargo/bin/diesel migration run --database-url='mysql://bottle_zoa:bottle_zoa@10.96.155.202/bottle_zoa'

~/.cargo/bin/diesel print-schema --database-url='mysql://bottle_zoa:bottle_zoa@10.96.155.202/bottle_zoa'

echo "Remember to remove update_at and create_at fields in tabls in schema.rs"
