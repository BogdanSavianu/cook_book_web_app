## Dev Test

```sh
# Test for model
cargo watch -q -c -w src/ -x 'test model_db_ -- --test-threads=1 --nocapture'

# Test for web
cargo watch -q -c -w src/ -x 'test web_ -- --test-threads=1 --nocapture'
```


## Dev Web
```sh
cargo watch -q -c -w src/ -x 'run -- ../frontend/web-folder'
```
