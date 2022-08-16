#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# pip install "fastapi[all]"

from fastapi import FastAPI
from pydantic import BaseModel

class Request(BaseModel):
    test: list[int]

server = FastAPI()

@server.post("/test")
async def insertion_sort_endpoint(request: Request):
    import time
    threads = []
    start_single = time.perf_counter()
    ordered = insertion_sort(request.test)
    end_single = time.perf_counter()
    import threading
    for thnum in range(200):
        x = threading.Thread(target=thread_core, args=(request.test,thnum))
        x.start()
        threads.append(x)
    for thread in threads:
        thread.join()
    end_threading = time.perf_counter()
    print("Total computation: "+str(end_threading-start_single))
    print("Single computation: "+str(end_single-start_single))
    print("Thread computation: "+str(end_threading-end_single))
    return {"result": ordered}

def thread_core(values, thread_num):
    print("Starting thread "+str(thread_num))
    insertion_sort(values)
    print("Ending thread thread "+str(thread_num))

def insertion_sort(values_raw):
    values = values_raw.copy()
    for x in range(1, len(values)):
        for j in reversed(range(x)):
            if values[j + 1] < values[j]:
                swap = values[j + 1]
                values[j+1] = values[j]
                values[j] = swap
    return values


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser(description='Server to sort using insertion sort')

    parser.add_argument('-p', '--port', dest="port", action='store', type=int, default=49163,
                        help='the port accepting requests from customers')
    parser.add_argument('-s', '--server', dest="host", action='store', default="127.0.0.1",
                        help='the host accepting requests from customers')

    args = parser.parse_args()
    import uvicorn
    if args.host:
        uvicorn.run(server, host=args.host, port=int(args.port))
    else:
        uvicorn.run(server, port=int(args.port))