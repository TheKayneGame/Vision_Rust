using Microsoft.Win32;
using System;
using System.Collections.Generic;
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
        public MainWindow()
        {
            InitializeComponent();
        }

        private void Button_Click_1(object sender, RoutedEventArgs e)
        {

        }

        private void Path_image_Click(object sender, RoutedEventArgs e)
        {
            OpenFileDialog openFileDialog = new OpenFileDialog();
            if (openFileDialog.ShowDialog() == true)
                MessageBox.Show(openFileDialog.FileName);
            path_image_text.Text = openFileDialog.FileName;

        }

        private void Load_image_Click(object sender, RoutedEventArgs e)
        {
           
            var uri = new Uri(path_image_text.Text);
            var bitmap = new BitmapImage(uri);

            image.Source = bitmap;
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
            //Set output of program to be written to process output stream
            rustprocess.StartInfo.RedirectStandardOutput = true;
            //Optional
            rustprocess.StartInfo.WorkingDirectory = strWorkingDirectory;
            //Start the process
            rustprocess.Start();
            //Get program output
            string strOutput = rustprocess.StandardOutput.ReadToEnd();
            //Wait for process to finish
            rustprocess.WaitForExit();
            return strOutput;
        }
    }
}
