# Icons
## baby
<div>Icons made by <a href="https://www.flaticon.com/authors/freepik" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com/" title="Flaticon">www.flaticon.com</a></div>

# Runner
<div>Icons made by <a href="https://www.flaticon.com/authors/freepik" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com/" title="Flaticon">www.flaticon.com</a></div>

# Additional
<a target="_blank" href="/icons/set/volume-level">Volume level</a>, <a target="_blank" href="/icons/set/stacked-organizational-chart">Stacked Organizational Chart</a> and other icons by <a target="_blank" href="https://icons8.com">Icons8</a>

# Design

## change table
currently, we have the following rows:

change_type => The change type
vpin_id => The versionpin we are acting on
dist_id => The target distribution id
pkgcoord_id => The package coordinates id (not used)
display => opaque display text

In save we 
collect the vpin_id and the dist_id

changes we want to be able to make
| change             | changing           |
| ------------------ | ------------------ |
| ChangeDistribution | dist_id            |
| NewDistribution    | dist_id PkgCoords  |
| ChangePkgcoord     | vpin_id  PkgCoords |
| ChangeWiths        | vpin_id withs      |

We could cache the changes like so:
enum ChangeCandidate {
    ChangeDistribition{vpin_id: IdType, new_dist_id: IdType }
    NewDistribution{package:String, version: String, level: String, role:String, platform:String, site:String}
    ChangePkgCoord{vpin_id: IdType, version: String, level: String, role:String, platform:String, site:String}
    ChangeWiths{vpin_id: IdType, withs: Vec<String>}
}

and store changes like so;
C