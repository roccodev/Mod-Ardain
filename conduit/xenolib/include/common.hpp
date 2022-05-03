// "Monolith Math"?
// Some math related functions
namespace mm {
    template<typename NumType = int> struct Pnt {
        NumType x;
        NumType y;
    };

    // A RGBA color
    struct Col4 {
        float r;
        float g;
        float b;
        float alpha;
    };
}

// "Framework"?
// These are common functions between XC2 and XCDE.
namespace fw {
    namespace debug {
        void drawSquare2D(mm::Pnt<int> const& pnt1, mm::Pnt<int> const& pnt2, mm::Col4 const& color);
    }
}
