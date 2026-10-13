-- `lake env lean tests/Axioms.lean`: axioms of every final theorem (LB/Main.lean)
-- and of the main intermediate results.  Expected: propext, Classical.choice,
-- Quot.sound only (no sorryAx, no Lean.ofReduceBool / native_decide).
import LB
#print axioms LB.thm_phi_leaves
#print axioms LB.thm_phi_cost
#print axioms LB.thm_phi_depth
#print axioms LB.thm_phi_V
#print axioms LB.count_identities
#print axioms LB.add_is_closure
#print axioms LB.lem_world_L
#print axioms LB.lem_world_A
#print axioms LB.duplicate_accounting
#print axioms LB.local_inequality
#print axioms LB.correct_leaf
#print axioms LB.thm_key_path
#print axioms LB.thm_key
#print axioms LB.cor_combined
#print axioms LB.prop_phiopt
#print axioms LB.Phi_in_family
#print axioms LB.V_attained
#print axioms LB.sharp_for_minimum
#print axioms LB.CTree.leaves_le_two_pow_depth
#print axioms LB.children_correct
#print axioms LB.pruning
#print axioms LB.V_is_min_depth
#print axioms LB.family_chain
#print axioms LB.prop_phiopt_stronger
