import euklid
import euklid_rs
import sys


def get_attributes(module_name):
    attributes = []
    for module in sys.modules:
        if module.startswith(module_name):
            for classname in dir(sys.modules[module]):
                if not classname.startswith("_"):
                    cls = getattr(sys.modules[module], classname)

                    if type(cls) == type or "pybind11_type" in str(type(cls)):
                        for attr in dir(cls):
                            if not attr.startswith("_"):
                                attributes.append(f"{classname}.{attr}")
                        
                    elif "module" not in str(type(cls)):
                        attributes.append(f"{classname}")
    
    return set(attributes)

def get_table():
    a = get_attributes("euklid.")
    b = get_attributes("euklid_rs.")

    combined = a.union(b)

    chars = max([len(x) for x in combined])
    def space(x, char=" "):
        return char * (chars - len(x)) + x

    table = "``` diff\n"
    table += f"#|{space('Attribute')} | euklid | euklid_rs |\n"
    table += f"#|{space('', '-')}-|--------|----------|\n"


    for attribute in sorted(list(combined)):
        if attribute not in b:
            table += "-"
        elif attribute not in a:
            table += "+"
        else:
            table += "#"
            
        table += f"|{space(attribute)} | "

        
        if attribute in a:
            table += "✓"
        else:
            table += "✗"
        
        table += "      | "
        if attribute in b:
            table += "✓"
        else:
            table += "✗"

        table += "        |\n"
    
    table += "```"
    
    return table


if __name__ == "__main__":
    with open("support.md", "w+") as outfile:
        outfile.write("# Support Table\n\n")
        outfile.write(get_table())