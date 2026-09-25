# meta-continuum-image

Top-level image recipes. `continuum-image-phase0-minimal.bb` (to be
added) assembles kernel + base layers per
`../../manifests/phase-0-minimal-image.manifest` into the Phase 0
exit-criteria bootable image. Later phases add
`continuum-image-phase1.bb` etc., each including the prior phase's
package set plus that phase's additions, per the "superset, never a
rewrite" principle in the parent README.
