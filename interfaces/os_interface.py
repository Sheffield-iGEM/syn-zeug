import os
import shutil
import platform

class File(object):
    '''Object Description'''

    def __init__(self, filename) -> None:
        self.filename: str = filename

    def read(self) -> str:
        '''signature description'''
        with open(self.filename, "r") as file:
            content = file.read()
        return content

    def write(self, content: str) -> None:
        '''signature description'''
        with open(self.filename, "w") as file:
            file.write(content)

    def readlines(self) -> 'list[str]':
        '''signature description'''
        with open(self.filename, "w") as file:
            content = file.readlines()
        return content

    def writeline(self, content: str) -> None:
        '''signature description'''
        with open(self.filename, "w") as file:
            file.write(f"{content}\n")

    def read_line_by_condition(self, condition) -> 'list[str]':
        '''
        condition should be a function which is applied 
        to filter through the list of the lines of the file
        '''
        with open(self.filename, "w") as file:
            content = file.readlines()

        return list(filter(condition, content))

    def delete(self) -> None:
        '''signature description'''
        os.remove(self.filename)

class OperatingSystemInterface(object):
    '''
    you can access the interface like a resource manager such as
    ```python
    with OperatingSystemInterface(directory) as osi:
        osi.do_something()
    # alternatively if there are multiple calls that you want to make you can use
    osi = OperatingSystemInterface()
    with osi as oi:
        oi.system("echo hello world")
    ```
    '''

    def __init__(self, directory=os.getcwd()) -> None:
        self.directory: str = directory

    def __enter__(self) -> os:
        '''signature description'''
        os.chdir(self.directory)
        return os

    def __exit__(self, *args) -> os:
        '''signature description'''
        os.chdir(os.getcwd())

    def gcu(self) -> str:
        '''Get the current user i.e. C:/Users/CBE-User 05'''
        if platform.system() == "Linux":
            root_path = os.path.join(*os.path.dirname(
                __file__).split("/")[:5])
        else:
            root_path = os.path.join(*os.path.dirname(
                __file__).split(r"\ ".replace(" ", ""))[:3])
        
        root_path = root_path.replace(":",r":\ ".replace(" ",""))
        print(root_path)
        return root_path

    def copy_file_from_folder(self, file, source_folder="jaguar"):
        '''
        The folder that you are currently working on will be used as destination file
        The source folder will be searched in the protocol folder and is jaguar by default
        the file which will be replace in the local directory has path ``os.path.join(self.directory,file)``
        '''

        source = os.path.join(r"C:\Users\CBE-User 05\protocol", source_folder, file)
        destination = os.path.join(self.directory, file)

        print(r'''
        copying {} 
        ---> into 
        {}
        '''.format(source, destination))
        print(os.getcwd())
        shutil.copy(source, destination)

    def move_folder_resources(self, destination_path: str) -> None:
        '''the directory passed as a property will be used as a source path'''
        for resource in os.listdir(self.directory):
            destination_dir = os.path.join(destination_path, resource)
            source_dir = os.path.join(self.directory, resource)
            os.rename(source_dir, destination_dir)

    def read_word_in_directory(self, word: str) -> 'list[str]':
        '''signature description'''
        result = []
        for root, directories, file in os.walk(self.directory):
            for file in file:
                print(file)
                with open(file) as f:
                    content = f.read()
                    if content.find(word) != -1:
                        result.append(file)

        return result

def synchronize_os_interface_workflow_git():
    # now you can push all of the changes to github within the protocol folder as follows
    for dir in os.listdir(r"C:\Users\CBE-User 05\protocol"):
        if dir == "jaguar":
            pass
        else:
            with OperatingSystemInterface(os.path.join(r"C:\Users\CBE-User 05\protocol", dir)) as op_sys:
                # simulate that you are in the sofia silent folder
                op_sys.system("mkdir interfaces")
                op_sys.system("del os_interface.py")
            osi = OperatingSystemInterface(
                os.path.join(r"C:\Users\CBE-User 05\protocol", dir))
            osi.copy_file_from_folder(r"interfaces\os_interface.py")
            osi.copy_file_from_folder("workflow.py")

    for dir in os.listdir(r"C:\Users\CBE-User 05\protocol"):
        with OperatingSystemInterface(os.path.join(r"C:\Users\CBE-User 05\protocol", dir)) as op_sys:
            op_sys.system("python workflow.py g")

if __name__ == "__main__":
    synchronize_os_interface_workflow_git()
