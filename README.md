
# Design Notes

## Todos - p1
- [ ] add selected distro/platform to current level
- [ ] modify current pin's platform / role / etc
- [ ] add packages.xml generation / installation
- [ ] add install table to db
- [ ] add install distro to packybara to populate table 
- [ ] modify packages tree to use install as source of data
- [ ] add support for linked shows
- [ ] update client to use thread for queries
- [ ] update history view to present with updates, pin installs, etc 
- [ ] add support for change sets
- [ ] add support for servistry configuration
- [ ] copy withs between pins
- [ ] show update locking for duration of change / install
- [ ] change notification
- [ ] grpc python api 
- [ ] add banner for when other user makes change during current session 
## Todos - p2
- [ ] add persistent configuration
- [ ] add theming 
- [ ] roll back changes
- [ ] diff changes
- [ ] add support for multiple server sites via server/service
- [ ] add support for multi selection in with package dropdown
- [ ] add reload in with package dropdown
- [ ] add caching (server and/or client side)

## add versionpin dialog

select roles
select sequence / shots
select locations

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