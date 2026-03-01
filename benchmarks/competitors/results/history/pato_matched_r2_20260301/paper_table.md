# Head-to-Head Wall Time (ms)

- Run ID: `pato_matched_r2_20260301`
- Suite: `large`
- Operation: `consistency`
- Metric: wall time in milliseconds (container-level; parse + reasoning + startup)

| Ontology | tableauxx | hermit | openllet | elk | jfact | pellet |
|---|---:|---:|---:|---:|---:|---:|
| pato.owl | **3326** | 11901 | TO | 6524 | TO | 724357 |

Legend: **best wall time**, TO=timeout, NA=not available, NC=non-comparable parser/runtime incompatibility, KIL=interrupted cleanup kill, ORPH=container orphan cleanup, FAIL=hard runtime failure.
