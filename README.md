# Protocol Benchmarks

Below some details about the tests, graphs generated by Criterion will be included.

A conclusion drawn from pure (de)serialization times would suggest that one should always use Cap'n Proto. Somewhat surprising, to me, is that the process of building the object to serialize is slower in Cap'n Proto vs protobuf, see the last graph below.

Running `cargo bench` in this repository can be done to explore these results on your system.

The structs used to test these can be viewed in the [`protos`](protos) directory, summarized here for easy access:

```capnp
@0xd1f1c56543bdb3ac;

struct Basic {
    id @0 :UInt64;
}

struct Complex {
    name @0 :Text;
    basic @1 :Basic;
    reference @2 :Text;
}
```

```protobuf
syntax = "proto2";
option optimize_for = SPEED;

package bench;

message Basic {
    required uint64 id = 1;
}

message Complex {
    required string name = 1;
    required Basic basic = 2;
    required string reference = 3;
}
```

## Basic Read

![Basic Read](.criterion/basic_read/summary/new/violin_plot.svg)

## Basic Write

![Basic Write](.criterion/basic_write/summary/new/violin_plot.svg)

## Complex Read

![Complex Read](.criterion/complex_read/summary/new/violin_plot.svg)

## Complex Write

![Complex Write](.criterion/complex_write/summary/new/violin_plot.svg)

### Complex Build

This benchmark is the same as the write above, but includes the actual struct construction.

![Complex Build](.criterion/complex_build/summary/new/violin_plot.svg)
