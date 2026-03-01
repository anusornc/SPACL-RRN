# Head-to-Head Wall Time (ms)

- Run ID: `pato_matched_r1_20260301`
- Suite: `large`
- Operation: `consistency`
- Metric: wall time in milliseconds (container-level; parse + reasoning + startup)

| Ontology | tableauxx | hermit | openllet | elk | jfact | pellet |
|---|---:|---:|---:|---:|---:|---:|
| chebi.owl | NA | NA | NA | NA | NA | NA |
| doid.owl | NA | NA | NA | NA | NA | NA |
| go-basic.owl | NA | NA | NA | NA | NA | NA |
| pato.owl | **3137** | 14457 | TO | 7042 | TO | 752048 |
| uberon.owl | NA | NA | NA | NA | NA | NA |

Legend: **best wall time**, TO=timeout, NA=not available, NC=non-comparable parser/runtime incompatibility, KIL=interrupted cleanup kill, ORPH=container orphan cleanup, FAIL=hard runtime failure.
