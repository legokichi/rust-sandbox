#!/bin/bash
cat log.txt |grep chat |awk -F "\t" '{print $4 "\t" $5}'
