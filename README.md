
# Design Notes

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

what would a table look like for this?

| change_type        | context                                                                    | old value                                                         | new value                                                           |
| ------------------ | -------------------------------------------------------------------------- | ----------------------------------------------------------------- | ------------------------------------------------------------------- |
| ChangeDistribution | (level: facility, role: any, platform:any, site: any, package: maya)       | 2018.2.3                                                          | 2018.2.4                                                            |
| NewDistribution    | (level: dev01, role: any, platform:any, site: any, package: maya)          |                                                                   | 2019.0.0                                                            |
| ChangeWiths        | (level: dev01, role: any, platform:any, site: any, package: maya) 2018.2.3 |                                                                   | [gcc,xerces,modelpublish]                                           |
| ChangePkgCoord     | dist_id: 12345                                                             | (level: dev01, role: any, platform:any, site: any, package: maya) | (level: dev01, role: model, platform:any, site: any, package: maya) |