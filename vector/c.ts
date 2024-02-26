class MyClass {
    member: string;
    constructor(member: string) {
        this.member = member;
    }
    method1() {
        // Some code here
    }
    method2() {
        // Some code here
    }
}

function printClassStructure(obj: any) {
    console.log("object |");
    // Print members
    for (let prop of Object.getOwnPropertyNames(obj)) {
        if (typeof obj[prop] !== 'function') {
            console.log(`        \\ MEMBER: ${prop}`);
        }
    }
    // Print methods
    for (let method of Object.getOwnPropertyNames(obj.constructor.prototype)) {
        if (method !== 'constructor' && typeof obj.constructor.prototype[method] === 'function') {
            console.log(`        \\ METHOD: ${method}`);
        }
    }
}

printClassStructure(new MyClass('example'));