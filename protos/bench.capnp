@0xd1f1c56543bdb3ac;

struct Basic {
    id @0 :UInt64;
}

struct Complex {
    name @0 :Text;
    basic @1 :Basic;
    reference @2 :Text;
}
