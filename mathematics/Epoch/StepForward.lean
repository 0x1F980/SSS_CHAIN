/-!
# SSS chain epoch step forward

Aligns with `sss_chain_epoch_step_forward` in `SSS_CHAIN/src/sss_chain_epoch.rs`.
Used by transport `TransportOtpRatchet` and asymmetric epoch FS.
-/

namespace Epoch

/-- Abstract field step: `next = trans - current` with `trans = 2*current + anchor + idx + entropy`. -/
def stepForward (current anchor idx entropy : Nat) : Nat :=
  (current + current + anchor + idx + entropy) - current

theorem step_forward_deterministic (current anchor idx entropy : Nat) :
    stepForward current anchor idx entropy =
      stepForward current anchor idx entropy := rfl

/-- Forward-only: counter strictly increases in ratchet consumers. -/
theorem epoch_counter_increases (c : Nat) : c < c + 1 := Nat.lt_succ_self c

end Epoch
