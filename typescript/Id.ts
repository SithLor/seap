interface id {
    id: `${string}-${string}-${string}-${string}-${string}`
    getId(): `${string}-${string}-${string}-${string}-${string}`
}
class Id implements id {
    id:`${string}-${string}-${string}-${string}-${string}`
    constructor(){
        
        this.id = crypto.randomUUID()
    }
    getId():`${string}-${string}-${string}-${string}-${string}`{
        return this.id
    }
}

