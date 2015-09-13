var searchIndex = {};
searchIndex['composite'] = {"items":[[0,"","composite","An Entity Component System for game development.",null,null],[3,"ComponentData","","ComponentData knows which entities have which components.",null,null],[12,"components","","components holds the components owned by a certain entity.",0,null],[12,"families","","Family to list of entities.",0,null],[3,"EntityManager","","The `EntityManager` type manages all the entities.",null,null],[12,"data","","",1,null],[3,"ComponentAdder","","Used by `EntityManager` to add components to an Entity.",null,null],[0,"component_presence","","Optional presence of a component.",null,null],[4,"ComponentPresence","composite::component_presence","Type `ComponentPresence` represents the optional presence of a component.",null,null],[13,"Comp","","",2,null],[13,"None","","",2,null],[11,"fmt","","",2,{"inputs":[{"name":"componentpresence"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"unwrap","","Returns the inner Component if there is one.\nPanics otherwise.",2,{"inputs":[{"name":"componentpresence"}],"output":{"name":"t"}}],[11,"as_ref","","",2,{"inputs":[{"name":"componentpresence"}],"output":{"name":"componentpresence"}}],[11,"as_mut","","",2,{"inputs":[{"name":"componentpresence"}],"output":{"name":"componentpresence"}}],[11,"has_it","","",2,{"inputs":[{"name":"componentpresence"}],"output":{"name":"bool"}}],[11,"lacks_it","","",2,{"inputs":[{"name":"componentpresence"}],"output":{"name":"bool"}}],[11,"deref","","",2,{"inputs":[{"name":"componentpresence"}],"output":{"name":"t"}}],[11,"deref_mut","","",2,{"inputs":[{"name":"componentpresence"}],"output":{"name":"t"}}],[0,"family","composite","",null,null],[5,"matcher","composite::family","Checks if the given `components` meet the requirements of a family.",null,{"inputs":[{"name":"reqtuple"},{"name":"vec"}],"output":{"name":"bool"}}],[6,"ReqTuple","","Has two vecs: The first one is a list of must have components,\nthe second is a list of forbidden components.",null,null],[6,"FamilyMap","","Matches each family to its list of required and forbidden components.",null,null],[8,"FamilyDataHolder","","Implemented by the `families!` macro, has the knowledge of all families requirements.",null,null],[10,"new","","",3,{"inputs":[{"name":"familydataholder"}],"output":{"name":"self"}}],[10,"all_families","","",3,{"inputs":[{"name":"familydataholder"}],"output":{"name":"familymap"}}],[0,"builder","composite","Implements `Build` an interface to define prototypes",null,null],[0,"event","","Event handling system.",null,null],[3,"EventManager","composite::event","Holds all events.",null,null],[8,"EventDataHolder","","Implemented automatically by the `events!` macro.",null,null],[10,"as_type","","",4,{"inputs":[{"name":"eventdataholder"}],"output":{"name":"str"}}],[11,"new","","",5,{"inputs":[{"name":"eventmanager"}],"output":{"name":"eventmanager"}}],[11,"global_events","","Global Events are events meant to be checked outside of a `Behavior`",5,{"inputs":[{"name":"eventmanager"}],"output":{"name":"drain"}}],[11,"of_type","","Returns all events of a given type.",5,{"inputs":[{"name":"eventmanager"},{"name":"str"}],"output":{"name":"hashmap"}}],[11,"for_behavior_of","","Given the events a behavior listens to, and the entity it will process, return the relevant events.",5,{"inputs":[{"name":"eventmanager"},{"name":"vec"},{"name":"entity"}],"output":{"name":"vec"}}],[11,"push_for","","Push the given event, for the given entity to process at some point.",5,{"inputs":[{"name":"eventmanager"},{"name":"entity"},{"name":"holder"}],"output":null}],[11,"push_global","","Pushes the given event as a global one.",5,{"inputs":[{"name":"eventmanager"},{"name":"holder"}],"output":null}],[0,"behavior","composite","Management and creation of behaviors (a.k.a Systems in most ECSs).",null,null],[3,"BehaviorManager","composite::behavior","Keeps track of behavior and what family and events they care about.",null,null],[8,"BehaviorData","","",null,null],[10,"family","","",6,{"inputs":[{"name":"behaviordata"}],"output":{"name":"str"}}],[10,"events","","",6,{"inputs":[{"name":"behaviordata"}],"output":{"name":"vec"}}],[8,"Behavior","","",null,null],[10,"process","","",7,{"inputs":[{"name":"behavior"},{"name":"vec"},{"name":"entity"},{"name":"componentdata"},{"name":"eventmanager"}],"output":null}],[11,"new","","",8,null],[11,"run","","",8,{"inputs":[{"name":"behaviormanager"},{"name":"entitymanager"},{"name":"eventmanager"}],"output":null}],[6,"Entity","composite","Type Entity is simply an ID used as indexes.",null,null],[8,"EntityDataHolder","","This is a marker trait to be used by the `components!` macro.",null,null],[10,"new","","",9,{"inputs":[{"name":"entitydataholder"}],"output":{"name":"self"}}],[10,"match_families","","Takes a map of all the defined families,\nand returns the families that match this entity.",9,{"inputs":[{"name":"entitydataholder"},{"name":"familymap"}],"output":{"name":"vec"}}],[10,"set_families","","Sets the families this entity belongs to to `families`",9,{"inputs":[{"name":"entitydataholder"},{"name":"vec"}],"output":null}],[10,"belongs_to_family","","",9,{"inputs":[{"name":"entitydataholder"},{"name":"str"}],"output":{"name":"bool"}}],[10,"families","","Gets the known families this ent belongs to.",9,{"inputs":[{"name":"entitydataholder"}],"output":{"name":"vec"}}],[8,"Component","","This trait marks a struct as a component. (Automatically handled by macro `components!`)",null,null],[10,"add_to","","Adds self to the specified entity. Called by the `EntityManager`",10,{"inputs":[{"name":"component"},{"name":"entity"},{"name":"componentdata"}],"output":null}],[11,"new","","",0,{"inputs":[{"name":"componentdata"}],"output":{"name":"componentdata"}}],[11,"get","","",0,{"inputs":[{"name":"componentdata"},{"name":"entity"}],"output":{"name":"option"}}],[11,"get_mut","","",0,{"inputs":[{"name":"componentdata"},{"name":"entity"}],"output":{"name":"option"}}],[11,"create_component_data_for","","",0,{"inputs":[{"name":"componentdata"},{"name":"entity"}],"output":null}],[11,"set_family_relation","","",0,{"inputs":[{"name":"componentdata"},{"name":"str"},{"name":"entity"}],"output":null}],[11,"members_of","","",0,{"inputs":[{"name":"componentdata"},{"name":"str"}],"output":{"name":"vec"}}],[11,"any_member_of","","",0,{"inputs":[{"name":"componentdata"},{"name":"str"}],"output":{"name":"bool"}}],[11,"index","","",0,{"inputs":[{"name":"componentdata"},{"name":"entity"}],"output":{"name":"d"}}],[11,"index_mut","","",0,{"inputs":[{"name":"componentdata"},{"name":"entity"}],"output":{"name":"d"}}],[11,"new","","Creates a new EntityManager",1,{"inputs":[{"name":"entitymanager"}],"output":{"name":"entitymanager"}}],[11,"new_entity","","Creates a new entity, assigning it an unused ID, returning that ID for further use.",1,{"inputs":[{"name":"entitymanager"}],"output":{"name":"entity"}}],[11,"add_component","","Sets up for insertion of a single component.",1,{"inputs":[{"name":"entitymanager"},{"name":"c"}],"output":{"name":"componentadder"}}],[11,"add_component_to","","Adds the specified component to the entity.",1,{"inputs":[{"name":"entitymanager"},{"name":"entity"},{"name":"c"}],"output":null}],[11,"new","","",11,{"inputs":[{"name":"componentadder"},{"name":"c"},{"name":"componentdata"},{"name":"familymap"}],"output":{"name":"componentadder"}}],[11,"to","","",11,{"inputs":[{"name":"componentadder"},{"name":"entity"}],"output":null}],[14,"families!","","defines the families and which components an entity must (or musn't) have to belong to it.",null,null],[14,"prototypes!","","See the [builder module documentation](/ecs/builder/) for information on this macro.",null,null],[14,"events!","","",null,null],[14,"behaviors!","","",null,null],[14,"components!","","The components macro defines all the structs and traits that manage\nthe component part of the ECS.",null,null]],"paths":[[3,"ComponentData"],[3,"EntityManager"],[4,"ComponentPresence"],[8,"FamilyDataHolder"],[8,"EventDataHolder"],[3,"EventManager"],[8,"BehaviorData"],[8,"Behavior"],[3,"BehaviorManager"],[8,"EntityDataHolder"],[8,"Component"],[3,"ComponentAdder"]]};
searchIndex['lib'] = {"items":[[0,"","lib","An Entity Component System for game development.",null,null],[3,"ComponentData","","ComponentData knows which entities have which components.",null,null],[12,"components","","components holds the components owned by a certain entity.",0,null],[12,"families","","Family to list of entities.",0,null],[3,"EntityManager","","The `EntityManager` type manages all the entities.",null,null],[12,"data","","",1,null],[3,"ComponentAdder","","Used by `EntityManager` to add components to an Entity.",null,null],[0,"component_presence","","Optional presence of a component.",null,null],[4,"ComponentPresence","lib::component_presence","Type `ComponentPresence` represents the optional presence of a component.",null,null],[13,"Comp","","",2,null],[13,"None","","",2,null],[11,"fmt","","",2,{"inputs":[{"name":"componentpresence"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"unwrap","","Returns the inner Component if there is one.\nPanics otherwise.",2,{"inputs":[{"name":"componentpresence"}],"output":{"name":"t"}}],[11,"as_ref","","",2,{"inputs":[{"name":"componentpresence"}],"output":{"name":"componentpresence"}}],[11,"as_mut","","",2,{"inputs":[{"name":"componentpresence"}],"output":{"name":"componentpresence"}}],[11,"has_it","","",2,{"inputs":[{"name":"componentpresence"}],"output":{"name":"bool"}}],[11,"lacks_it","","",2,{"inputs":[{"name":"componentpresence"}],"output":{"name":"bool"}}],[11,"deref","","",2,{"inputs":[{"name":"componentpresence"}],"output":{"name":"t"}}],[11,"deref_mut","","",2,{"inputs":[{"name":"componentpresence"}],"output":{"name":"t"}}],[0,"family","lib","",null,null],[5,"matcher","lib::family","Checks if the given `components` meet the requirements of a family.",null,{"inputs":[{"name":"reqtuple"},{"name":"vec"}],"output":{"name":"bool"}}],[6,"ReqTuple","","Has two vecs: The first one is a list of must have components,\nthe second is a list of forbidden components.",null,null],[6,"FamilyMap","","Matches each family to its list of required and forbidden components.",null,null],[8,"FamilyDataHolder","","Implemented by the `families!` macro, has the knowledge of all families requirements.",null,null],[10,"new","","",3,{"inputs":[{"name":"familydataholder"}],"output":{"name":"self"}}],[10,"all_families","","",3,{"inputs":[{"name":"familydataholder"}],"output":{"name":"familymap"}}],[0,"builder","lib","Implements `Build` an interface to define prototypes",null,null],[0,"event","","Event handling system.",null,null],[3,"EventManager","lib::event","Holds all events.",null,null],[8,"EventDataHolder","","Implemented automatically by the `events!` macro.",null,null],[10,"as_type","","",4,{"inputs":[{"name":"eventdataholder"}],"output":{"name":"str"}}],[11,"new","","",5,{"inputs":[{"name":"eventmanager"}],"output":{"name":"eventmanager"}}],[11,"global_events","","Global Events are events meant to be checked outside of a `Behavior`",5,{"inputs":[{"name":"eventmanager"}],"output":{"name":"drain"}}],[11,"of_type","","Returns all events of a given type.",5,{"inputs":[{"name":"eventmanager"},{"name":"str"}],"output":{"name":"hashmap"}}],[11,"for_behavior_of","","Given the events a behavior listens to, and the entity it will process, return the relevant events.",5,{"inputs":[{"name":"eventmanager"},{"name":"vec"},{"name":"entity"}],"output":{"name":"vec"}}],[11,"push_for","","Push the given event, for the given entity to process at some point.",5,{"inputs":[{"name":"eventmanager"},{"name":"entity"},{"name":"holder"}],"output":null}],[11,"push_global","","Pushes the given event as a global one.",5,{"inputs":[{"name":"eventmanager"},{"name":"holder"}],"output":null}],[0,"behavior","lib","Management and creation of behaviors (a.k.a Systems in most ECSs).",null,null],[3,"BehaviorManager","lib::behavior","Keeps track of behavior and what family and events they care about.",null,null],[8,"BehaviorData","","",null,null],[10,"family","","",6,{"inputs":[{"name":"behaviordata"}],"output":{"name":"str"}}],[10,"events","","",6,{"inputs":[{"name":"behaviordata"}],"output":{"name":"vec"}}],[8,"Behavior","","",null,null],[10,"process","","",7,{"inputs":[{"name":"behavior"},{"name":"vec"},{"name":"entity"},{"name":"componentdata"},{"name":"eventmanager"}],"output":null}],[11,"new","","",8,null],[11,"run","","",8,{"inputs":[{"name":"behaviormanager"},{"name":"entitymanager"},{"name":"eventmanager"}],"output":null}],[6,"Entity","lib","Type Entity is simply an ID used as indexes.",null,null],[8,"EntityDataHolder","","This is a marker trait to be used by the `components!` macro.",null,null],[10,"new","","",9,{"inputs":[{"name":"entitydataholder"}],"output":{"name":"self"}}],[10,"match_families","","Takes a map of all the defined families,\nand returns the families that match this entity.",9,{"inputs":[{"name":"entitydataholder"},{"name":"familymap"}],"output":{"name":"vec"}}],[10,"set_families","","Sets the families this entity belongs to to `families`",9,{"inputs":[{"name":"entitydataholder"},{"name":"vec"}],"output":null}],[10,"belongs_to_family","","",9,{"inputs":[{"name":"entitydataholder"},{"name":"str"}],"output":{"name":"bool"}}],[10,"families","","Gets the known families this ent belongs to.",9,{"inputs":[{"name":"entitydataholder"}],"output":{"name":"vec"}}],[8,"Component","","This trait marks a struct as a component. (Automatically handled by macro `components!`)",null,null],[10,"add_to","","Adds self to the specified entity. Called by the `EntityManager`",10,{"inputs":[{"name":"component"},{"name":"entity"},{"name":"componentdata"}],"output":null}],[11,"new","","",0,{"inputs":[{"name":"componentdata"}],"output":{"name":"componentdata"}}],[11,"get","","",0,{"inputs":[{"name":"componentdata"},{"name":"entity"}],"output":{"name":"option"}}],[11,"get_mut","","",0,{"inputs":[{"name":"componentdata"},{"name":"entity"}],"output":{"name":"option"}}],[11,"create_component_data_for","","",0,{"inputs":[{"name":"componentdata"},{"name":"entity"}],"output":null}],[11,"set_family_relation","","",0,{"inputs":[{"name":"componentdata"},{"name":"str"},{"name":"entity"}],"output":null}],[11,"members_of","","",0,{"inputs":[{"name":"componentdata"},{"name":"str"}],"output":{"name":"vec"}}],[11,"any_member_of","","",0,{"inputs":[{"name":"componentdata"},{"name":"str"}],"output":{"name":"bool"}}],[11,"index","","",0,{"inputs":[{"name":"componentdata"},{"name":"entity"}],"output":{"name":"d"}}],[11,"index_mut","","",0,{"inputs":[{"name":"componentdata"},{"name":"entity"}],"output":{"name":"d"}}],[11,"new","","Creates a new EntityManager",1,{"inputs":[{"name":"entitymanager"}],"output":{"name":"entitymanager"}}],[11,"new_entity","","Creates a new entity, assigning it an unused ID, returning that ID for further use.",1,{"inputs":[{"name":"entitymanager"}],"output":{"name":"entity"}}],[11,"add_component","","Sets up for insertion of a single component.",1,{"inputs":[{"name":"entitymanager"},{"name":"c"}],"output":{"name":"componentadder"}}],[11,"add_component_to","","Adds the specified component to the entity.",1,{"inputs":[{"name":"entitymanager"},{"name":"entity"},{"name":"c"}],"output":null}],[11,"new","","",11,{"inputs":[{"name":"componentadder"},{"name":"c"},{"name":"componentdata"},{"name":"familymap"}],"output":{"name":"componentadder"}}],[11,"to","","",11,{"inputs":[{"name":"componentadder"},{"name":"entity"}],"output":null}],[14,"families!","","defines the families and which components an entity must (or musn't) have to belong to it.",null,null],[14,"prototypes!","","See the [builder module documentation](/ecs/builder/) for information on this macro.",null,null],[14,"events!","","",null,null],[14,"behaviors!","","",null,null],[14,"components!","","The components macro defines all the structs and traits that manage\nthe component part of the ECS.",null,null]],"paths":[[3,"ComponentData"],[3,"EntityManager"],[4,"ComponentPresence"],[8,"FamilyDataHolder"],[8,"EventDataHolder"],[3,"EventManager"],[8,"BehaviorData"],[8,"Behavior"],[3,"BehaviorManager"],[8,"EntityDataHolder"],[8,"Component"],[3,"ComponentAdder"]]};
initSearch(searchIndex);
