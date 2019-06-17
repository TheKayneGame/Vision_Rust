using Microsoft.Win32;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

namespace VisionGui
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    /// 


    public partial class MainWindow : Window
    {
        const string rust_location = "vision/vison8b.exe";
        const string dice_arg = "--dice";
        const string licenceplate_arg = "--license";

        const string image_location = "vision/";


        enum modes {
            [Description("--dice")]
            dice,
            [Description("--license")]
            license
        };

        modes mode;

        

        public MainWindow()
        {
            InitializeComponent();
        }


        private void Path_image_Click(object sender, RoutedEventArgs e)
        {
            OpenFileDialog openFileDialog = new OpenFileDialog();
            if (openFileDialog.ShowDialog() == true)
            {
                path_image_text.Text = openFileDialog.FileName; 
            }
        }

        private void Load_image_Click(object sender, RoutedEventArgs e)
        {
            Change_image(path_image_text.Text);

        }


        private string run_rust(String Command, String CommandParameters)
        {
            //Create process
            Process rustprocess = new Process();
            //strCommand is path and file name of command to run
            rustprocess.StartInfo.FileName = Command;
            //strCommandParameters are parameters to pass to program
            rustprocess.StartInfo.Arguments = CommandParameters;
            rustprocess.StartInfo.UseShellExecute = false;
            rustprocess.StartInfo.CreateNoWindow = true;
            //Set output of program to be written to process output stream
            rustprocess.StartInfo.RedirectStandardOutput = true;
            //Optional
            //rustprocess.StartInfo.WorkingDirectory = strWorkingDirectory;
            //Start the process
            rustprocess.Start();
            //Get program output
            string strOutput = rustprocess.StandardOutput.ReadToEnd();
            //Wait for process to finish
            rustprocess.WaitForExit();
            return strOutput;
        }

        private void Run_Click(object sender, RoutedEventArgs e)
        {


            string test = run_rust("visionb8.exe", "--" + mode.ToString() + " " + path_image_text.Text);
            Change_image(mode.ToString() + "_end_result.bmp");
            MessageBox.Show(test);
        }

        private void Change_image(string path)
        {
            var uri = new Uri(path_image_text.Text);
            var bitmap = new BitmapImage(uri);
            image.Source = bitmap;
        }

        private void mode_change(object sender, RoutedEventArgs e)
        {
            var button = sender as RadioButton;
            MessageBox.Show(button.Name.ToString());
            switch (button.Name.ToString())
            {
                case "license":
                    mode = modes.license;
                    break;
                case "dice":
                    mode = modes.dice;
                    break;

            }
           
        }
    }
}
