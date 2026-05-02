#!/bin/bash
echo "Restoring dvdrental database..."
# -U postgres specifies the user, -d dvdrental specifies the target DB
# The tar file is mounted into the container's init directory
pg_restore -U postgres -d dvdrental /docker-entrypoint-initdb.d/dvdrental.tar
echo "Restore complete."
