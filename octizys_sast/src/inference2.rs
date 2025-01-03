trait Inference {
    type Type;
    type Expression;
    type Error;

    fn partial_evaluation(
        &self,
        type_: &Self::Type,
    ) -> Result<Self::Type, Self::Error>;
    fn unification(
        &self,
        first: &Self::Type,
        second: &Self::Type,
    ) -> Self::Error;
    fn check(
        &self,
        expression: &Self::Expression,
        type_: &Self::Type,
    ) -> Result<(), Self::Error>;
    fn infer(
        &self,
        expression: &Self::Expression,
    ) -> Result<&Self::Type, Self::Error>;
}
