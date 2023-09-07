#!/bin/bash
cat log.txt |grep player |awk -F "\t" '{print $4 "\t" $5 "\t" $6 "\t" $7 "\t" $8}'
