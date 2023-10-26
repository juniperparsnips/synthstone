Cookbook ROM format...


addresses of recipe IDs should be aligned to 16 bits
i.e. the hex of the address must end in 0

```
address   ||  value                                        ||  Comments
--------- ||-----------------------------------------------||---------------------------------------
<id>|0    ||  <type> | <output count> | <ing. list len>    ||  Type is like crafting table / furnace
<id>|<i>  ||  <slot #> | <item id>                         ||  

```


Serialization of Recipe IDs
...
