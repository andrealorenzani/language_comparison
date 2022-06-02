#!/usr/bin/env python
# -*- coding: utf-8 -*-

import random
values = [x for x in range(1, 10000)]

random.shuffle(values)
print('curl localhost:8080/ -d "test={}"'.format(values))