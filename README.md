# Octizys
A functional language



# Compiler Stages

- Parse to CST (Concrete Syntax Tree):
    + It has all the comments.
    + Maintains identifiers in the tree.
- CST Checks: 
    + Check that every definition is declared.
    + Check that every declaration has a definition.
    + Check that only one definition/declaration for a single identifier.
    + Find the dependency's for every definition, then we know what 
        imports we need.
- Transform from CST to SAST (Sugared Abstract Syntax Tree):
    + Transform to nameless representation ready for type inference.
- Type inference.
    ModuleContext = {MapNameToVariable, MapVariableToName, }
    InferenceContext = {Imports:Context,Module:mut Context, Local: mut Context,constraints}
    infer : sast_expression -> InferenceContext -> Either core_type_withHoles Errors
    check : sast_expression -> sast_type -> InferenceContext -> Either core_type_withHoles Errors
    solve_constraints : Context -> Option Errors
- Core transformation.
