#!/bin/bash

while read p; do
 wget "https://data.commoncrawl.org/${p}"
done < cc-index.paths
