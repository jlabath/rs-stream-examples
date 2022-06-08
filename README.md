# rs-stream-examples

samples of use of streams for concurrent computation

### sample run

    cargo run
    
### sample output

```
buffer_unordered concurrent execution stream examples.
tweak the buffer size to see impact, current buffer size: 3
Result of constant 1 second sleep with buffer size 3 took 4.009463875s and was [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
Result of random from 0 to 1 second sleep with buffer size 3 took 2.644905416s and was [3, 4, 2, 1, 5, 6, 9, 7, 8, 10]
```
