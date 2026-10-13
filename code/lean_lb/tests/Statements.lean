-- `lake env lean tests/Statements.lean`: the fully elaborated statements of the
-- final theorems (all implicit and instance arguments shown), for the audit.
import LB
set_option pp.funBinderTypes true
#check @LB.thm_phi_leaves
#check @LB.thm_phi_cost
#check @LB.thm_phi_depth
#check @LB.thm_phi_V
#check @LB.thm_key
#check @LB.thm_key_path
#check @LB.cor_combined
#check @LB.prop_phiopt
#check @LB.correct_leaf
#check @LB.local_inequality
#check @LB.duplicate_accounting
#check @LB.count_identities
#check @LB.V_attained
