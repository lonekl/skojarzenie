pub enum Instruction {

    LoadAStack(usize),
    LoadBStack(usize),
    StoreAStack(usize),

    IntegerAdd,
    IntegerSubtract,
    IntegerMultiply,
    IntegerDivide,

    FloatAdd,
    FloatSub,
    FloatMultiply,
    FloatDivide,

    FloatSinus,
    FloatCoSinus,
    FloatTangent,
    FloatCoTangent,
    FloatArcSinus,
    FloatArcCoSinus,
    FloatArcTangent,
    FloatArcCoTangent,
    FloatInverseSinus,
    FloatInverseCoSinus,
    FloatInverseTangent,
    FloatInverseCoTangent,
    FloatInverseArcSinus,
    FloatInverseArcCoSinus,
    FloatInverseArcTangent,
    FloatInverseArcCoTangent,

}



pub struct Function {

    pub name: String,
    stack: usize,

}
