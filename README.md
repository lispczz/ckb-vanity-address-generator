This is a ckb vanity address generator written in rust.   

## Usage

In the following example, we are searching for an address with prefix `ckb1qyqzhang`(All ckb addresses start with `ckb1qyq`), and we finally find that a private key `0xc17ec148a0b16495937f04c388f2d2a22f2afc88cf0918ec611973de4df61b1d` can producer an address `ckb1qyqzhangajf6qvj2yjwls04tzze4zdz5lvcqhxsmut` which satisfies the condition.   

```
 $ cargo run --release ckb1qyqzhang
    Finished release [optimized] target(s) in 8.98s
     Running `target/release/ckb-vanity-address-generator ckb1qyqzhang`
checking prefix ckb1qyqzhang
count: 600000	elapsed: 0.06min	speed: 179479.51/s	progress(est): 3.58%	left(est): 1.50min
count: 1200000	elapsed: 0.12min	speed: 170672.74/s	progress(est): 7.15%	left(est): 1.52min
count: 1800000	elapsed: 0.18min	speed: 163547.16/s	progress(est): 10.73%	left(est): 1.53min
count: 2400000	elapsed: 0.25min	speed: 160459.99/s	progress(est): 14.31%	left(est): 1.49min
count: 2980000	elapsed: 0.33min	speed: 151023.72/s	progress(est): 17.76%	left(est): 1.52min
count: 3580000	elapsed: 0.40min	speed: 148949.45/s	progress(est): 21.34%	left(est): 1.48min
count: 4180000	elapsed: 0.47min	speed: 146857.32/s	progress(est): 24.91%	left(est): 1.43min
count: 4790000	elapsed: 0.54min	speed: 148187.11/s	progress(est): 28.55%	left(est): 1.35min
count: 5390000	elapsed: 0.61min	speed: 148329.57/s	progress(est): 32.13%	left(est): 1.28min
count: 5990000	elapsed: 0.68min	speed: 147362.72/s	progress(est): 35.70%	left(est): 1.22min
count: 6600000	elapsed: 0.75min	speed: 147337.87/s	progress(est): 39.34%	left(est): 1.15min
result:
privkey:	0xc17ec148a0b16495937f04c388f2d2a22f2afc88cf0918ec611973de4df61b1d
address:	ckb1qyqzhangajf6qvj2yjwls04tzze4zdz5lvcqhxsmut
```

